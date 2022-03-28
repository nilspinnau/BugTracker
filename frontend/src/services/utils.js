import React from "react";
import { useNavigate } from "react-router-dom";

import Button from "react-bootstrap/Button";

export function request(method="POST", url, body, headers) {

    let requestOptions = {
        method: method,
        headers: headers,
    };
    if (method !== "GET") {
        requestOptions.body = JSON.stringify(body);
    }
    return fetch(url, requestOptions)
        .then(response => {
            const isJson = response.headers.get("content-type")?.includes("application/json");
            const data = isJson && response.json();

            // check for error response
            if (!response.ok) {
                // get error message from body or default to response status
                return {
                    ok: false,
                    data: data || response.status
                };
            }

            return {
                ok: true,
                data: data
            };
        })
        .catch(error => {
            return {
                ok: false,
                data: error
            }
        });
}

export function RedirectTo(props) {
    let navigate = useNavigate();
    function handleClick() {
        if (props.path) {
            navigate(props.path);
        } else {
            navigate("/");
        }
    }
    
    return (
        <Button style={{ "marginRight": "1rem" }} onClick={handleClick}>
                {props.message}
        </Button>
    );
}  


