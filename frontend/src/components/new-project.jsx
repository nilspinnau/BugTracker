import React from "react";
import "./index.css";

// bootstrap
import Container from "react-bootstrap/Container";
import Button from "react-bootstrap/Button";
import Modal from "react-bootstrap/Modal";
import Form from 'react-bootstrap/Form'
import useAuth from "./auth";

// own


class ProjectForm extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            item: {
                title: "",
                team: "",
                description: ""
            }
        };

        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    handleSubmit = (event) => {
        this.props.onHide();
        event.preventDefault();
        let url = "/api/new_project";
        // we need some headers as well
        let headers = {
            "Content-Type": "application/json"
        };
        let requestOptions = {
            method: "POST",
            headers: headers,
            body: this.state.item
        };
        // now fetch our data
        fetch(url, requestOptions)
            .then(response => console.log(response))
            .catch(error => console.log(error));
    }

    handleChange(event) {
        this.setState({ 
            item: {...this.state.item, [event.target.name]: event.target.value}
        });
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
                                <Form.Control name="title" type="text" placeholder="Project title" onChange={this.handleChange}/>
                            </Form.Group>

                            <Form.Group className="mb-3" controlId="formBasicEmail">
                                <Form.Label>Team</Form.Label>
                                <Form.Control name="team" type="list" placeholder="What team should be working on it?" onChange={this.handleChange}/>
                            </Form.Group>

                            <Form.Group className="mb-3" controlId="exampleForm.ControlTextarea1">
                                <Form.Label>Description</Form.Label>
                                <Form.Control name="description" as="textarea" rows={5} placeholder="Description for the project" onChange={this.handleChange}/>
                            </Form.Group>
                        
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

function NewProject(props) {
    const [modalShow, setModalShow] = React.useState(false);
    const { authed } = useAuth();

    return (
        <>
        <Button 
            {...props}
            onClick={() => {
                if (authed) {
                    setModalShow(true)
                }
            }}>
            Create new Project
        </Button>

        <ProjectForm
            {...props}
            show={modalShow}
            onHide={() => setModalShow(false)}
        />
        </>
    );
}


export default NewProject;