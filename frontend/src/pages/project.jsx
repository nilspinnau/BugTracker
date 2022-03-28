import React from "react";
import "./index.css";
import { useSearchParams } from "react-router-dom";

// bootstrap
import Container from "react-bootstrap/Container";
import Alert from "react-bootstrap/Alert";

// own
import { RedirectTo } from "../services/utils";

function ProjectPage() {
    let [searchParams, _] = useSearchParams();
    return <Project id={searchParams.get("id")} />
}

class Project extends React.Component {
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
    }

    __get() {
        let id = this.state.id;
        const searchParams = new URLSearchParams();
        searchParams.append("id", id);
        let url = "/project?"+searchParams.toString();
        // we need some headers as well
        let headers = {
            "Authentication": sessionStorage.getItem("auth_token")
        };
        let requestOptions = {
            method: "GET",
            headers: headers
        };
        // now fetch our data
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

    handleChange(event) {
        this.setState({ 
            item: {...this.state.item, [event.target.name]: event.target.value}
        });
    }

    render() {
        if (!this.state.item || Object.keys(this.state.item).length === 0) {
            return this.missingProject();
        } else {
            return(
                <Container>
                
                </Container>
            );
        }
    }
}

export default ProjectPage;