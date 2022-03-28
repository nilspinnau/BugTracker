import React from "react";
import { useSearchParams } from "react-router-dom";
import "./index.css";

// bootstrap
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";
import Alert from "react-bootstrap/Alert";
import Container from "react-bootstrap/Container";


// own
import { NewItem } from "../components";
import {RedirectTo, request} from "../services/utils";




function ItemPage() {
    let [searchParams, _] = useSearchParams();
    return <Item id={searchParams.get("id")} />
}

class Item extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            disabled: false,
            item: {},
            form_context: {
                assignee_list: [],
                project_list: []
            }
        }

        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    __get() {
        let id = this.props.id;
        let searchParams = new URLSearchParams;
        searchParams.set("id", id);
        let url = "/item?"+searchParams.toString();
        let headers = {};
        let requestOptions = {
            method: "GET",
            headers: headers
        }
        fetch(url, requestOptions)
            .then(response => response.json())
            .then(data => {
                this.setState({item: data})
            })
            .catch(error => console.log(error));
    }

    componentDidMount() {
        // GET request using fetch with error handling
        this.__get();
    }
    
    componentDidUpdate(prevProps) {
        // Typical usage (don't forget to compare props):
        if (this.props.id !== prevProps.id) {
          this.__get();
        }
    }

    getEffort() {
        return(
            <Container>
                <Form.Label>Effort</Form.Label><br />
                <Form.Select disabled={this.state.disabled} placeholder={this.state.item.effort} name="effort" onChange={this.handleChange}>
                    <option value="0">Select effort of issue</option>
                    <option value="0">&#60; 1 day</option>
                    <option value="1">&#60; 2 days</option>
                    <option value="2">&#60; 3 days</option>
                    <option value="3">&#60; 1 week</option>
                    <option value="4">&#60; 1 month</option>
                    <option value="5">&#62; 1 month</option>
                </Form.Select><br /><br />
            </Container>
        );
    }

    getSeverity() {
        return(
            <Container>
                <Form.Label>Severity</Form.Label><br />
                <Form.Select disabled={this.state.disabled} placeholder={this.state.item.severity} name="severity" onChange={this.handleChange}>
                    <option value="0">Minor</option>
                    <option value="1">Moderate</option>
                    <option value="2">Major</option>
                    <option value="3">Critical</option>
                </Form.Select><br /><br />
            </Container>
        );
    }

    getStatus() {
        return(
            <Container>
                <Form.Label>Status</Form.Label><br />
                <Form.Select disabled={this.state.disabled} placeholder={this.state.item.status} name="status" onChange={this.handleChange}>
                    <option value="0">Select status of issue</option>
                    <option value="0">Open</option>
                    <option value="1">In Progress</option>
                    <option value="2" disabled>Closed</option>
                </Form.Select><br /><br />
            </Container>
        );
    }
    
    missingItem() {
        
        return (
            <Container>
                <Alert variant="danger">
                    <Alert.Heading>Oh snap! No issue found with id {this.props.id}!</Alert.Heading>
                        <p>
                            Try a different id or create a new issue.
                        </p>
                    <RedirectTo message="Go Home" path="/"/>
                    <NewItem variant="primary" />
                </Alert>
            </Container>
        );
    }

    handleChange(event) {
        this.setState({ 
            item: {...this.state.item, [event.target.name]: event.target.value}
        });
    }

    handleSubmit = (event) => {
        this.props.onHide();
        event.preventDefault();
        let url = "/update_item";
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
        fetch(url, requestOptions)
            .then(response => response.status)
            .catch(error => console.log(error));
    }

    render() {
        if (!this.state.item || Object.keys(this.state.item).length === 0) {
            return this.missingItem();
        } else {
            return(
                <Container>
                <Form onSubmit={this.handleSubmit}>
                    <Form.Group disabled className="mb-3" controlId="formBasicEmail">
                        <Form.Label>Title</Form.Label>
                        <Form.Control name="title" type="text" placeholder="Issue short description" onChange={this.handleChange}/>
                    </Form.Group>
    
                    <Form.Group disabled className="mb-3" controlId="formBasicEmail">
                        <Form.Label>Project</Form.Label>
                        <Form.Control name="project" type="datalist" list={this.state.form_context.project_list} placeholder="In which project did the issue occur" onChange={this.handleChange}/>
                    </Form.Group>
    
                    <Form.Group disabled={this.state.disabled} className="mb-3" controlId="formBasicEmail">
                        <Form.Label>Assignee</Form.Label>
                        <Form.Control name="assignee" type="datalist" list={this.state.form_context.assignee_list} placeholder="Who should be assigned for this?" onChange={this.handleChange}/>
                    </Form.Group>
    
                    {this.getSeverity()}
                    {this.getEffort()}
    
                    <Form.Group disabled={this.state.disabled} className="mb-3" controlId="exampleForm.ControlTextarea1">
                        <Form.Label>Description</Form.Label>
                        <Form.Control name="description" as="textarea" rows={5} onChange={this.handleChange}>{this.state.item.description}</Form.Control>
                    </Form.Group>
    
                    {this.getStatus()}
        
                    <Container style={{ "textAlign": "right" }}>
                        <Button onClick={this.setState({disabled: true})}>
                            Edit Issue
                        </Button>
                        <Button onClick={this.updateItem} type="submit">
                            Submit
                        </Button>
                    </Container>
                </Form>
    
                <Container id="comments">
                </Container>
                </Container>
            );
        }
    }
}


export default ItemPage;