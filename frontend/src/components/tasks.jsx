import React from "react";
import Container from "react-bootstrap/Container";

// should include only the teams doing?
class TaskHistory extends React.Component {
    constructor(props) {
        super(props)
        this.state = {}
    }

    getRecentTasks() {
        // http post request to server to collect the most recent tasks (issues movement)
    }

    render() {
        return(
            <Container>
        
            </Container>
        );
    }
} 


// A number of tasks (5) concerning only the user, sorted after importance (severity)
class Todos extends React.Component {
    constructor(props) {
        super(props)
        this.state = {}
    }

    getTodos() {
        // http post request to server to collect tasks,  (newest or oldest????) status open/in_progress sorted by severity
    }

    render() {
        return(
            <Container>
        
            </Container>
        );
    }
}



export default {Todos, TaskHistory};