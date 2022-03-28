import React from "react";
import "./index.css";

// bootstrap
import ListGroup from "react-bootstrap/ListGroup";


import { ItemCard } from "../components";
import Container from "react-bootstrap/esm/Container";


function ItemList(props, to, items, filter, value) {

    items = items.filter((x) => x[filter] === value);

    if (items.length === 0) {
        return(
            <Container>
                <ListGroup variant="flush" className="listgroup-scroll">
                    <ListGroup.Item>{<ItemCard to={to+"?id=none"} {...props} id="none" created={new Date().toLocaleString()} />}</ListGroup.Item>
                </ListGroup>
            </Container>
        );
    } else {
        return(
            <Container>
                <ListGroup variant="flush" className="listgroup-scroll">
                {
                    items.map( (x) =>
                        <ListGroup.Item>{<ItemCard to={to+"?id="+x.id} {...props} id={x.id} title={x.title} created={x.created} />}</ListGroup.Item>)
                }
                </ListGroup>
            </Container>
        );
    }
}


export default ItemList;