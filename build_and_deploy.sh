#!/bin/bash

PROFILE="default"
REGION="us-east-1"
SUBNET_IDS="subnet-02bb44a3f461bba21"
SECURITY_GROUP_IDS="sg-03b5bc583cc303d0f"
VPC_ID="vpc-06f4c9d49639d1048"
LOCAL_MOUNT_PATH="/mnt/rootly"
FUNCTION_NAMES=()

# Help function
usage() {
    echo "Usage: $0 [options]"
    echo "  -p, --profile  AWS profile to use (default: $PROFILE)"
    echo "  -r, --region   AWS region (default: $REGION)"
    echo "  -h, --help     Display this help message"
    exit 1
}

# Parsing logic
while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--profile)
            PROFILE="$2"
            shift 2 # Move past key and value
            ;;
        -r|--region)
            REGION="$2"
            shift 2
            ;;
        -vpc)
            VPC_ID="$2"
            shift 2
            ;;
        -subnet-ids)
            SUBNET_IDS="$2"
            shift 2
            ;;
        -security-group-ids)
            SECURITY_GROUP_IDS="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *) # Handle unknown options or positional args
            echo "Unknown option: $1"
            usage
            ;;
    esac
done
echo "Starting Build and Deployment Script..."
echo "-----------------------------------Building--------------------------------------"
export AWS_PROFILE=$PROFILE
export AWS_REGION=$REGION
# Configuration - Update these!
ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
ROLE_ARN="arn:aws:iam::${ACCOUNT_ID}:role/dev-fileservice-rag-dispatcher-lambda-role"
API_NAME="Rootly"
ADD_LOGS_FUNC="addLogs"
GET_LOGS_FUNC="getLogs"
INIT_FUNC="init"
LOGS_RESOURCE_PATH="logs"
STAGE_NAME="prod"
EFS_NAME="RootlyStorage"
EFS_ACCESS_POINT_ARN=""
LAMBDA_LISTS=($ADD_LOGS_FUNC $GET_LOGS_FUNC $INIT_FUNC)

EFS_ID=$(aws efs describe-file-systems --region $REGION \
    --query "FileSystems[?Name=='$EFS_NAME'].FileSystemId" \
    --output text)

if [ -z "$EFS_ID" ] || [ "$EFS_ID" == "None" ]; then
    echo "EFS not found. Creating it now..."
    echo "Creating File System..."
    EFS_ID=$(aws efs create-file-system \
        --performance-mode generalPurpose \
        --throughput-mode bursting \
        --region $REGION \
        --tags Key=Name,Value=$EFS_NAME \
        --query 'FileSystemId' --output text)
    echo "Created File System: $EFS_ID"
    echo "Waiting for File System to become available..."
    while true; do
        STATE=$(aws efs describe-file-systems --file-system-id $EFS_ID --query 'FileSystems[0].LifeCycleState' --output text)
        if [ "$STATE" == "available" ]; then
            echo "File System is ready!"
            break
        fi
        echo "Current state: $STATE... checking again in 5s"
        sleep 5
    done
    aws efs create-mount-target \
            --file-system-id $EFS_ID \
            --subnet-id $SUBNET_IDS \
            --security-groups $SECURITY_GROUP_IDS \
            --region $REGION
    echo "Mount target created for $SUBNET"

    echo "Creating EFS Access Point (Required for Lambda)..."
    # Lambda needs an Access Point to define the user/directory it uses
    AP_ID=$(aws efs create-access-point \
        --file-system-id $EFS_ID \
        --posix-user Uid=1000,Gid=1000 \
        --root-directory "Path=/rootly,CreationInfo={OwnerUid=1000,OwnerGid=1000,Permissions=777}" \
        --region $REGION \
        --query 'AccessPointId' --output text)
    EFS_ACCESS_POINT_ARN=$(aws efs describe-access-points --file-system-id $EFS_ID --query "AccessPoints[0].AccessPointArn" --output text)
else
    echo "EFS $EFS_NAME already exists with ID: $EFS_ID"
    EFS_ACCESS_POINT_ARN=$(aws efs describe-access-points --file-system-id $EFS_ID --query "AccessPoints[0].AccessPointArn" --output text)
fi

echo "Checking/Creating Lambda Function..."
for FUNC_NAME in "${LAMBDA_LISTS[@]}"; do
  cargo lambda build --manifest-path ./aws/functions/$FUNC_NAME/Cargo.toml --release --arm64 --bin $FUNC_NAME
  cargo lambda deploy --manifest-path ./aws/functions/$FUNC_NAME/Cargo.toml --iam-role $ROLE_ARN -r $REGION -p $PROFILE --subnet-ids $SUBNET_IDS \
  --security-group-ids $SECURITY_GROUP_IDS --binary-name $FUNC_NAME --memory 1024 --timeout 30
done
for FUNC_NAME in "${LAMBDA_LISTS[@]}"; do
  while true; do
    STATUS=$(aws lambda get-function --function-name $FUNC_NAME --query 'Configuration.State' --output text)
    if [ "$STATUS" == "Active" ]; then
      echo "Lambda function $FUNC_NAME is active!"
      aws lambda update-function-configuration --function-name $FUNC_NAME \
          --file-system-configs "Arn=$EFS_ACCESS_POINT_ARN,LocalMountPath=$LOCAL_MOUNT_PATH" \
          --environment "Variables={STORAGE_PATH='$LOCAL_MOUNT_PATH'}">/dev/null
      break
    fi
    echo "Current status of $FUNC_NAME: $STATUS... checking again in 5s"
    sleep 5
  done
done

echo "Checking/Creating REST API..."
API_ID=$(aws apigateway get-rest-apis --query "items[?name=='$API_NAME'].id" --output text)

if [ -z "$API_ID" ]; then
    API_ID=$(aws apigateway create-rest-api --name "$API_NAME" --query 'id' --output text)
    echo "Created API with ID: $API_ID"
else
    echo "API $API_NAME already exists with ID: $API_ID"
fi

echo "Finding Root Resource..."
ROOT_ID=$(aws apigateway get-resources --rest-api-id $API_ID --query "items[?path=='/'].id" --output text)

echo "Checking/Creating Resource '/$LOGS_RESOURCE_PATH'..."
RES_ID=$(aws apigateway get-resources --rest-api-id $API_ID --query "items[?path=='/$LOGS_RESOURCE_PATH'].id" --output text)

if [ -z "$RES_ID" ]; then
    RES_ID=$(aws apigateway create-resource --rest-api-id $API_ID --parent-id $ROOT_ID --path-part $LOGS_RESOURCE_PATH --query 'id' --output text)
    echo "Created resource $LOGS_RESOURCE_PATH with ID: $RES_ID"
else
    echo "Resource $LOGS_RESOURCE_PATH already exists."
fi

echo "Setting up methods and Integration..."
# We use '|| true' to ignore errors if the method already exists
aws apigateway put-method --rest-api-id $API_ID --resource-id $RES_ID --http-method GET --authorization-type "NONE" >/dev/null 2>&1 || echo "Method GET already exists."
aws apigateway put-method --rest-api-id $API_ID --resource-id $RES_ID --http-method POST --authorization-type "NONE" >/dev/null 2>&1 || echo "Method POST already exists."

aws apigateway put-integration --rest-api-id $API_ID --resource-id $RES_ID --http-method GET \
    --type AWS_PROXY --integration-http-method POST \
    --uri "arn:aws:apigateway:$REGION:lambda:path/2015-03-31/functions/arn:aws:lambda:$REGION:$ACCOUNT_ID:function:$GET_LOGS_FUNC/invocations" >/dev/null 2>&1

aws apigateway put-integration --rest-api-id $API_ID --resource-id $RES_ID --http-method POST \
    --type AWS_PROXY --integration-http-method POST \
    --uri "arn:aws:apigateway:$REGION:lambda:path/2015-03-31/functions/arn:aws:lambda:$REGION:$ACCOUNT_ID:function:$ADD_LOGS_FUNC/invocations" >/dev/null 2>&1

echo "Granting Lambda Permissions..."
for FUNC_NAME in "${LAMBDA_LISTS[@]}"; do
  aws lambda add-permission --function-name $FUNC_NAME --statement-id apigateway-invoke \
      --action lambda:InvokeFunction --principal apigateway.amazonaws.com \
      --source-arn "arn:aws:execute-api:$REGION:$ACCOUNT_ID:$API_ID/*/*/$LOGS_RESOURCE_PATH" >/dev/null 2>&1 || echo "Permissions already set."
done


echo "Deploying API..."
aws apigateway create-deployment --rest-api-id $API_ID --stage-name $STAGE_NAME >/dev/null

echo "-------------------------------------------------------------------------------------------"
echo "Events Section"

INIT_RULE_ARN=$(aws events put-rule \
    --name "${INIT_FUNC}_RULE" \
    --schedule-expression "rate(1 minute)" \
    --state ENABLED \
    --region "$REGION" \
    --query 'RuleArn' \
    --output text)
echo "Rule created with ARN: $INIT_RULE_ARN"
echo "Adding permission to Lambda..."
aws lambda add-permission \
    --function-name "$INIT_FUNC" \
    --statement-id "${INIT_FUNC}_EventBridgeLambdaPermission" \
    --action "lambda:InvokeFunction" \
    --principal "events.amazonaws.com" \
    --source-arn "$INIT_RULE_ARN" \
    --region "$REGION"
echo "Setting Lambda as target..."
FUNCTION_ARN=$(aws lambda get-function --function-name "$INIT_FUNC" --query 'Configuration.FunctionArn' --output text)
aws events put-targets \
    --rule "${INIT_FUNC}_RULE" \
    --targets "Id"="1","Arn"="$FUNCTION_ARN" \
    --region "$REGION"

echo "-------------------------------------------------------------------------------------------"
echo "Deployment Complete!"
echo "URL: https://$API_ID.execute-api.$REGION.amazonaws.com/$STAGE_NAME/$LOGS_RESOURCE_PATH"