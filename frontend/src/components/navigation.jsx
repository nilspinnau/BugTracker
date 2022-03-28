import React from "react";
import { useNavigate, Link, NavLink } from "react-router-dom";

// bootstrap
import Container from "react-bootstrap/Container";
import { Navbar, Nav, NavDropdown } from 'react-bootstrap';


// own
import NewItem from "./new-item";
import NewProject from "./new-project";
import Button from "react-bootstrap/esm/Button";

import useAuth from "./auth";


// the sidebar could be build by the user itself maybe, where he can add new links to it e.g. stackoverflow or bugTracker internal links
// this has to be saved by the server again tho. Should be a feature far into the future
function Navigation() {
    const { authed, logout } = useAuth();
    const navigate = useNavigate();
  
    const handleLogout = () => {
        logout().then(() => {
            navigate("/");
        });
    };

    return (
        <>
            <Navbar bg="dark" variant="dark" expand="lg" sticky="top">
            <Container fluid>
                <Container fluid>
                    <Navbar.Brand>BugTracker</Navbar.Brand>
                    <Navbar.Toggle aria-controls="basic-navbar-nav" />
                </Container>
                <Navbar.Collapse id="basic-navbar-nav">
                    <Nav className="me-auto">
                        <NavDropdown title="+" id="basic-nav-dropdown">
                            <NavDropdown.Item>
                                <NewItem variant="none"/>
                            </NavDropdown.Item>
                            <NavDropdown.Item>
                                <NewProject variant="none"/>
                            </NavDropdown.Item>
                        </NavDropdown>

                        <Nav.Link as={NavLink} to="/">Home</Nav.Link>
                        <Nav.Link as={NavLink} to="/team">Team</Nav.Link>
                        <Nav.Link as={NavLink} to="/projects">Projects</Nav.Link>
                    </Nav>
                    {authed && <Button onClick={handleLogout}>Logout</Button>}
                </Navbar.Collapse>
            </Container>
            </Navbar>

            <Nav id="sidebarMenu" className="collapse d-lg-block sidebar collapse bg-white">
            <Container className="position-sticky">
                <Container className="list-group list-group-flush mx-3 mt-4">
                    <Link 
                        to="/chat" 
                        className="list-group-item list-group-item-action py-2 ripple" 
                        style={{color: "#6c757d"}}
                        >Chat</Link>
                    <Link 
                        to="/newsfeed"
                        className="list-group-item list-group-item-action py-2 ripple" 
                        style={{color: "#6c757d"}}
                        >Newsfeed</Link>
                </Container>
            </Container>
            </Nav>
        </>    
    );
}

export default Navigation;