use insta::assert_json_snapshot;
use parser::parse_html;

#[test]
fn basic_test_cases() {
    assert_json_snapshot!(parse_html("<html></html>"));

    assert_json_snapshot!(parse_html(
        "<html>
        <head>
        </head>
        <body>
        </body>
    </html>"
    ));

    assert_json_snapshot!(parse_html(
        "<html>
        <head>
            <meta charset=\"utf-8\" />
        </head>
        <body>
            <h1>Hello world</h1>
        </body>
    </html>"
    ));

    assert_json_snapshot!(parse_html(
        "<script>
        function a() {};
        console.log(a);
    </script>"
    ));

    assert_json_snapshot!(parse_html(
        "<style>
        .red {
            background-color: \"red\";
        }
    </style>"
    ));
}
