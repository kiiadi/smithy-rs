/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_sdk_glacier::{ByteStream, Credentials, Region};
use aws_smithy_client::test_connection::capture_request;
use aws_smithy_protocol_test::{assert_ok, validate_headers};

#[tokio::test]
async fn set_correct_headers() {
    let (conn, handler) = capture_request(None);
    let conf = aws_sdk_glacier::Config::builder()
        .region(Region::new("us-east-1"))
        .credentials_provider(Credentials::new("key", "secret", None, None, "test"))
        .build();

    let client = aws_sdk_glacier::Client::from_conf_conn(conf, conn);
    let _resp = client
        .upload_archive()
        .vault_name("vault")
        .body(ByteStream::from_path("tests/test-file.txt").await.unwrap())
        .send()
        .await;
    let req = handler.expect_request();
    assert_ok(validate_headers(
        &req,
        [
            (
                "x-amz-sha256-tree-hash",
                "2af02ea61585d13604b26ae314a99fc8e972d1f11daba655a68681843cfced9f",
            ),
            (
                "x-amz-content-sha256",
                "2af02ea61585d13604b26ae314a99fc8e972d1f11daba655a68681843cfced9f",
            ),
        ],
    ));
}
