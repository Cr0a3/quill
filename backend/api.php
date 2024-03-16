<?php

include "db.php";

$conn = connect();

if ( isset( $_GET["func"] ) ) {
    $func = $_GET["func"];

    if ( $func == "download" ) {
        $name = $_POST["name"];
        $version = $_POST["version"];

        echo "download $name v$version";
    }

    else if ( $func == "publish" ) {
        $data = file_get_contents('php://input');

        echo "publishing";
    }

    else if ( $func == "latest" ) {
        $name = $_POST["name"];

        echo "getting latest of $name";
    }

    else if ( $func == "view" ) {
        $name = $_POST["name"];

        echo "viewing $name";
    }
    
    else {
        echo "{error: \"invalid function name\"}";
    }

}

$conn->close();

?>