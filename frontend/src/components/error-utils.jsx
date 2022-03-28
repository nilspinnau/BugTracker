import React from "react";
import Container from "react-bootstrap/esm/Container";
import Alert from "react-bootstrap/Alert";

import { RedirectTo } from "../services/utils";

export function NotFound() {
    return (
        <Container>
            <Alert variant="danger">
                <Alert.Heading>404 Not Found</Alert.Heading>
                <RedirectTo message="Go Home" path="/"/>
            </Alert>
        </Container>
    );
}
    
export function Error(heading, message) {
    return (
        <Container>
            <Alert variant="danger">
                <Alert.Heading>{heading}</Alert.Heading>
                <p>{message}</p>
                <hr />
                <RedirectTo message="Go Home" path="/"/>
            </Alert>
        </Container>
    );
}