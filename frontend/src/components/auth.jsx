import React from "react";
import { useNavigate } from "react-router-dom";


const AuthContext = React.createContext();

// this is all only for client convenience. We check the clients authentication on the server, NOT on the client side
// this can all be overwritten essentially

export function AuthProvider({ children }) {
    const [authed, setAuthed] = React.useState(false);
    const navigate = useNavigate();
  
    
    function login(user) {
        let url = "/api/auth/login";
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
                    console.log("user is authenticated");
                    setAuthed(true);
                }
                setAuthed(true);
            })
            .catch(error => {
                console.log(error);
                setAuthed(false);
            })
    }
    function logout() {
        let url = "/api/auth/logout";
        let requestOptions = {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
        }
        return fetch(url, requestOptions)
            .then(response => {
                if (response.ok && response.status === 200) {
                    setAuthed(false);
                    navigate("/login");
                }
            })
            .catch(error => console.log(error))
    }

    return (<AuthContext.Provider value={
        {
            authed: authed, 
            login: login, 
            logout: logout
        }}>{children}</AuthContext.Provider>);
}

export default function useAuth() {
    return React.useContext(AuthContext);
}

