import React from "react";
import { Link } from "react-router-dom";
import "./index.css";

// bootstrap
import ListGroupItem from "react-bootstrap/esm/ListGroupItem";
import ListGroup from "react-bootstrap/ListGroup";
import Container from "react-bootstrap/Container";
import Button from "react-bootstrap/Button";
import Card from "react-bootstrap/Card";
import Alert from "react-bootstrap/Alert";
 
// own
import { RedirectTo, request } from "../services/utils";
import { NewProject } from "../components";
import { getProjects } from "../services/project/project-utils";

class Projects extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            projects: getProjects()
        }
    }

    item(itemJson) {
        return (
            <Card style={{width: "25rem" }}>
            <Card.Body>
                <Card.Title>{itemJson.title}</Card.Title>
                <Card.Subtitle className="mb-2 text-muted">{itemJson.team}</Card.Subtitle>
                <Card.Text>
                    {itemJson.description}
                </Card.Text>
                <Link to={"/projects?id="+this.props.id}>
                    <Button variant="primary" style={{marginTop: "0.5rem"}}>Show Details</Button>
                </Link>
            </Card.Body>
            </Card>
        )
    }
    
    // we list all projects here
    render() {
        if (this.state.projects.length > 0) {
            return (
                <Container>
                    <ListGroup>
                        <ListGroupItem>
                            {this.state.projects.map( (x) => this.item(x))}
                        </ListGroupItem>
                    </ListGroup>
                </Container>
            );
        } else {
            return(
                <Container>
                    <Alert variant="danger">
                        <Alert.Heading>Oh snap! No Project found</Alert.Heading>
                            <p>
                                There has yet to be a project created for it to be displayed here
                            </p>
                        <RedirectTo message="Go Home" path="/"/>
                        <NewProject variant="primary" />
                    </Alert>
                </Container>
            )
        }
    }
}

export default Projects;