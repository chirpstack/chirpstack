import { Link } from "react-router-dom";

import { Input, Typography, Button, Space } from "antd";

import { CreateApiKeyResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

interface IProps {
  createApiKeyResponse: CreateApiKeyResponse;
}

function ApiKeyToken(props: IProps) {
  return (
    <Space direction="vertical" style={{ width: "100%" }}>
      <Typography>
        <Typography.Paragraph>
          Use the following API token when making API requests. This token can be revoked at any time by deleting it.
          Please note that this token can only be retrieved once:
        </Typography.Paragraph>
      </Typography>
      <Input.TextArea rows={4} value={props.createApiKeyResponse.getToken()} />
      <Button type="primary">
        <Link to="../api-keys">Back</Link>
      </Button>
    </Space>
  );
}

export default ApiKeyToken;
