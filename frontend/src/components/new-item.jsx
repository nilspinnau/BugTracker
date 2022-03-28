import React from "react";
import "./index.css";

// bootstrap
import Container from "react-bootstrap/Container";
import Modal from "react-bootstrap/Modal";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";


// own
import { request } from "../services/utils";
import { getTeam } from "../services/team/team-utils";
import { getProjects } from "../services/project/project-utils";
import useAuth from "./auth";


class Item extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            item: {
                title: "",
                project: 0,
                assignee: "",
                severity: 0,
                effort: 0,
                description: "",
                status: 0
            },
            form_context: {
                project_list: getProjects(),
                assignee_lis: getTeam(-1)
            }
        };

        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    handleSubmit = (event) => {
        this.props.onHide();
        event.preventDefault();
        let url = "/api/new_item";
        // we need some headers as well
        let headers = {
            "Content-Type": "application/json",
            "Authorization": sessionStorage.getItem("auth_token")
        };
        let requestOptions = {
            method: "POST",
            headers: headers,
            body: this.state.item
        };
        // now fetch our data
        return fetch(url, requestOptions)
            .then(response => response.status)
            .catch(error => console.log(error));
    }

    handleChange(event) {
        this.setState({ 
            item: {...this.state.item, [event.target.name]: event.target.value}
        });
        if (event.target.name === "project") {
            this.setState({
                form_context: {...this.state.form_context, assignee_list: getTeam(event.target.value)}
            })
        } 
    }

    render() {
        return (
            <Modal
            {...this.props}
            size="lg"
            aria-labelledby="contained-modal-title-vcenter"
            centered
            >
                <Modal.Header>
                    <Modal.Title id="contained-modal-title-vcenter">Report a new Issue</Modal.Title>
                </Modal.Header>
    
                <Modal.Body>
                    <Container>
                        <Form>
                            <Form.Group className="mb-3" controlId="formBasicEmail">
                                <Form.Label>Title</Form.Label>
                                <Form.Control name="title" type="text" placeholder="Issue short description" onChange={this.handleChange}/>
                            </Form.Group>

                            <Form.Group className="mb-3" controlId="formBasicEmail">
                                <Form.Label>Project</Form.Label>
                                <Form.Control name="project" type="datalist" list={this.state.form_context.project_list} placeholder="In which project did the issue occur" onChange={this.handleChange}/>
                            </Form.Group>

                            <Form.Group className="mb-3" controlId="formBasicEmail">
                                <Form.Label>Assignee</Form.Label>
                                <Form.Control name="assignee" type="datalist" list={this.state.form_context.assignee_list} placeholder="Who should be assigned for this?" onChange={this.handleChange}/>
                            </Form.Group>

                            <Form.Label>Severity</Form.Label><br />
                            <Form.Select name="severity" onChange={this.handleChange}>
                                <option value="0">Select severity of issue</option>
                                <option value="0">Minor</option>
                                <option value="1">Moderate</option>
                                <option value="2">Major</option>
                                <option value="3">Critical</option>
                            </Form.Select><br /><br />

                            <Form.Label>Effort</Form.Label><br />
                            <Form.Select name="effort" onChange={this.handleChange}>
                                <option value="0">Select effort of issue</option>
                                <option value="0">&#60; 1 day</option>
                                <option value="1">&#60; 2 days</option>
                                <option value="2">&#60; 3 days</option>
                                <option value="3">&#60; 1 week</option>
                                <option value="4">&#60; 1 month</option>
                                <option value="5">&#62; 1 month</option>
                            </Form.Select><br /><br />

                            <Form.Group className="mb-3" controlId="exampleForm.ControlTextarea1">
                                <Form.Label>Description</Form.Label>
                                <Form.Control name="description" as="textarea" rows={5} placeholder="Describe the issue. If possible add reproducability" onChange={this.handleChange}/>
                            </Form.Group>

                            <Form.Label>Status</Form.Label><br />
                            <Form.Select name="status" onChange={this.handleChange}>
                                <option value="0">Select status of issue</option>
                                <option value="0">Open</option>
                                <option value="1">In Progress</option>
                                <option value="2" disabled>Closed</option>
                            </Form.Select><br /><br />
                        
                        <Container style={{textAlign: "right"}}>
                            <Button style={{marginRight: "0.5rem"}} variant="secondary" onClick={this.props.onHide}>Cancel</Button>
                            <Button variant="primary" type="Submit" onClick={this.handleSubmit}>Submit</Button>
                        </Container>
                        </Form>
                    </Container>     
                </Modal.Body>
            </Modal>
        );
    }
}

function NewItem(props) {
    const [modalShow, setModalShow] = React.useState(false);

    const { authed } = useAuth();

    return (
        <>
        <Button {...props} onClick={() => {
            if (authed) {
                setModalShow(true)
            }
        }}>
            Report new Issue
        </Button>

        <Item
            {...props}
            show={modalShow}
            onHide={() => setModalShow(false)}
        />
        </>
    );
}

export default NewItem;