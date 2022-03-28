import React from "react";

// bootstrap
import Button from "react-bootstrap/esm/Button";
import Form from "react-bootstrap/Form";
import Container from "react-bootstrap/esm/Container";


import useAuth from "../components/auth";
import { useNavigate } from "react-router-dom";


function Login() {

    const [user, setUser] = React.useState({
        username: "",
        password: ""
    });
    
    const { login } = useAuth();
    const navigate = useNavigate();

    const handleSubmit = (event) => {
        event.preventDefault();
        login(user).then(() => navigate("/"));
    }

    const handleChange = (event) => {
        event.preventDefault();
        setUser({...user, [event.target.name]: event.target.value});
    }

    return (
        <Container>
            <Form onSubmit={handleSubmit}>
                <Form.Group className="mb-3">
                    <Form.Label>Username</Form.Label>
                    <Form.Control name="username" type="text" placeholder="Enter username or email" onChange={handleChange}/>
                    <Form.Text className="text-muted"></Form.Text>
                </Form.Group>

                <Form.Group className="mb-3" controlId="formBasicPassword">
                    <Form.Label>Password</Form.Label>
                    <Form.Control name="password" type="password" placeholder="Enter password" onChange={handleChange}/>
                    <Form.Text className="text-muted"></Form.Text>
                </Form.Group>
            <Container style={{textAlign: "right"}}>
                <Button variant="primary" type="Submit" onClick={handleSubmit}>Submit</Button>
            </Container>
            </Form>
        </Container>     
    );
}

export default Login;
