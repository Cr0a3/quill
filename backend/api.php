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
        $name = $_POST["name"];

        echo "publishing $name";
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