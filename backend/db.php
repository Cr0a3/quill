<?php 
function connect() {
    // Create connection
    $conn = mysqli_connect("localhost", "admin", "pwd");

    // Check connection
    if (!$conn) {
        die("Connection failed: " . mysqli_connect_error());
    }

    return $conn;
}

function close($con) {
    $con->close();
}

function create_db($name) {
    $conn = connect();

    // insert db
    $sql = "CREATE DATABASE $name";
    if ($conn->query($sql) === TRUE) {
        return true;
    } else {
        die("Error while creating database: ". $conn->error);
        return false;
    }
}

function create_table($name, $sql_field) {
    $conn = connect();

    $sql = "CREATE TABLE $name ($sql_field)";
        
    if ($conn->query($sql) === TRUE) {
        return true;
    } else {
        die("Error while creating table: ". $conn->error);
        return false;
    }
}

?>