import React from "react";
import Button from "react-bootstrap/esm/Button";
import Container from "react-bootstrap/esm/Container";
import Modal from "react-bootstrap/Modal";
import Card from "react-bootstrap/Card";
import Form from "react-bootstrap/Form";
import FloatingLabel from "react-bootstrap/esm/FloatingLabel";

function NewsModal(props) {

    return (
        <Modal
        {...props}
        size="lg"
        aria-labelledby="contained-modal-title-vcenter"
        centered
        >
            <Modal.Header>
                <Modal.Title id="contained-modal-title-vcenter">News share</Modal.Title>
            </Modal.Header>

            <Modal.Body>
                <Container>
                    <FloatingLabel controlId="floatingTextarea2">
                        <Form.Control
                        as="textarea"
                        placeholder="Leave a comment here"
                        style={{ height: '100px' }}
                        />
                    </FloatingLabel>   
                </Container>
            </Modal.Body>

            <Modal.Footer>
                <Button variant="secondary" onClick={props.onHide}>Cancel</Button>
                <Button variant="primary" onClick={() => this.sendBugForm()}>Submit</Button>
            </Modal.Footer>
        </Modal>
    );
}

function AddNews() {
    const [modalShow, setModalShow] = React.useState(false);

    return (
        <Container>
        <Container>
        <Button style={{ "marginBottom": "1rem" }} variant="primary" onClick={() => setModalShow(true)}>
            + Share News
        </Button>
        </Container>
        
        <NewsModal 
            show={modalShow}
            onHide={() => setModalShow(false)}/>
        </Container>
    );
}

function News() {
    let items = [];

    return (
        <Container>
            {items.map((x) => {
                <Card bg="dark" text="light" style={{ "textAlign": "left", "marginBottom": "1rem", "font-size": "1rem" }}>
                <Card.Body>
                    <Card.Title>Author</Card.Title>
                    <Card.Text>Somebody wrote something</Card.Text>
                </Card.Body>
                </Card>
                })
            }
        </Container>
    );
}

function NewsHistory() {
    return(
        <>
        <News/>
        </>
    );
}

// users can post something here
class Newsfeed extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return(
            <Container>
                <AddNews/>
                <NewsHistory/>
            </Container>
        );
    }
}

export default Newsfeed;