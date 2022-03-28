import React from "react";
import "./index.css";
import { useSearchParams } from "react-router-dom";

// bootstrap
import ListGroup from "react-bootstrap/ListGroup";
import Container from "react-bootstrap/Container";

// own
import ItemList from "../components/item-list";


function TeamPage() {
    let [searchParams, ] = useSearchParams();
    return <Team id={searchParams.get("id")} />
}

class Team extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            id: this.props.id,
            items: []
        };

    } 

    __get() {
        let id = this.state.id;
        const searchParams = new URLSearchParams();
        searchParams.append("id", id);
        let url = "/team?"+searchParams.toString();
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
                this.setState({items: data})
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

    render() {
        // we can do better visually, but its not important now
        // we can also add another filter, for example not by stati but by effort. However that way building must be much more dynamic
       return (
            <Container>
                <ListGroup horizontal="sm">
                    <Container>OPEN<br/>
                        {ItemList(this.props, "/item", this.state.items, "status", 0)}
                    </Container>
                
                    <Container>IN-PROGRESS<br/>
                        {ItemList(this.props, "/item", this.state.items, "status", 1)}
                    </Container>

                    <Container>IN-REVIEW<br/>
                        {ItemList(this.props, "/item", this.state.items, "status", 2)}
                    </Container>

                    <Container>CLOSED<br/>
                        {ItemList(this.props, "/item", this.state.items, "status", 3)}
                    </Container>
                </ListGroup>
            </Container>
        );
    }
}

export default TeamPage;