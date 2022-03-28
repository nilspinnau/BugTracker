import React from "react";
import { Navigate, useNavigate, useLocation } from "react-router-dom";


// bootstrap
import Container from "react-bootstrap/esm/Container";
import Form from "react-bootstrap/esm/Form";
import Button from "react-bootstrap/esm/Button";

function useSetup() {
    const [setUp, setSetup] = React.useState(false);
  
    return {
        setUp,
        setup(user) {
            let url = "/api/setup";
            let requestOptions = {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(user)
            }
            return fetch(url, requestOptions)
                .then(response => {
                    if (response.ok && response.status === 200) {
                        setSetup(true);
                    }
                })
                .catch(_ => setSetup(false))
        }
    };
}

function Setup() {
    // this page is secured by the backend and is never reachable, if the bugtracker backend is already setup
    const { setUp, setup } = useSetup();
    const [user, setUser] = React.useState({
        id: 0,
        username: "admin",
        email: "admin",
        password: "admin",
        name: "admin",
        team: 0,
        rights: 5 // its an admin
    });

    const navigate = useNavigate();
    const location = useLocation();
    const handleChange = (event) => {
        event.preventDefault();
        setUser({...user, [event.target.name]: event.target.value});
    }

    const handleSubmit = (event) => {
        event.preventDefault();

        console.log(user);
        setup(user).then(() => {
            navigate("/");
        });
    }


    return !setUp ? (
        <Container>
            <h4>
                Setup an admin account:

            </h4><br />

        <Form onSubmit={handleSubmit}>
            <Form.Group className="mb-3">
                <Form.Label>Username</Form.Label>
                <Form.Control name="username" type="text" placeholder="Enter username" onChange={handleChange}/>
                <Form.Text className="text-muted"></Form.Text>
            </Form.Group>

            <Form.Group className="mb-3">
                <Form.Label>Email</Form.Label>
                <Form.Control name="email" type="text" placeholder="Enter email" onChange={handleChange}/>
                <Form.Text className="text-muted"></Form.Text>
            </Form.Group>

            <Form.Group className="mb-3">
                <Form.Label>Prename</Form.Label>
                <Form.Control name="prename" type="text" placeholder="Enter prename" onChange={handleChange}/>
                <Form.Text className="text-muted"></Form.Text>
            </Form.Group>
            
            <Form.Group className="mb-3">
                <Form.Label>Last Name</Form.Label>
                <Form.Control name="lastname" type="text" placeholder="Enter lastname" onChange={handleChange}/>
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
        </Container>)
        : (<Navigate to="/login" replace state={{ path: location.pathname }} />);
}

export default Setup;
