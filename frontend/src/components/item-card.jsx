import React from "react";
import { Link } from "react-router-dom";
import "./index.css";

// bootstrap
import Card from "react-bootstrap/Card";
import Button from "react-bootstrap/esm/Button";

import { RedirectTo } from "../services/utils";


function ItemCard(props) {
    if (props.id === "none") {
        return (
            <Card style={{width: "15rem" }}>
            <Card.Body>
                <Card.Title>ID '404': Not found</Card.Title>
                <Card.Subtitle className="mb-2 text-muted">{props.created}</Card.Subtitle>
                <Card.Text>This item does not exist.</Card.Text>
                <RedirectTo to="/" message="Go Back Home"/>
                <Link to={props.to}>
                    <Button variant="primary" style={{marginTop: "0.5rem"}}>Show Details</Button>
                </Link>
            </Card.Body>
            </Card>
        );
    }
    return (
        <Card style={{width: "15rem" }}>
        <Card.Body>
            <Card.Title>ID {props.id}: {props.title}</Card.Title>
            <Card.Subtitle className="mb-2 text-muted">{props.created}</Card.Subtitle>
            <Link to={props.to}>
                <Button variant="primary" style={{marginTop: "0.5rem"}}>Show Details</Button>
            </Link>
        </Card.Body>
        </Card>
    )
}


export default ItemCard;