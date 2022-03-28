import React from "react";
import "./index.css";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import {
    Navigation
} from "./components"

import { NotFound } from "./components/error-utils";

import {
    Team,
    Projects,
    Project,
    Item,
    Home,
    Newsfeed,
    Chat,
    Admin,
    Login,
    Setup
} from "./pages"
import PrivateRoute from "./components/private-route";

import { AuthProvider } from "./components/auth";



export default class App extends React.Component {

    render() {
        return(
            <React.StrictMode>
                <Router>
                    <AuthProvider>
                    <Navigation />
                    <Routes>
                        <Route path="/" element={
                            <PrivateRoute>
                                <Home />
                            </ PrivateRoute>} />
                        <Route path="/team" element={
                            <PrivateRoute>
                                <Team />
                            </ PrivateRoute>} />
                        <Route path="/chat" element={
                            <PrivateRoute>
                                <Chat />
                            </ PrivateRoute>} />
                        <Route path="/projects" element={
                            <PrivateRoute>
                                <Projects />
                            </ PrivateRoute>} />
                        <Route path="/project" element={
                            <PrivateRoute>
                                <Project />
                            </ PrivateRoute>} />
                        <Route exact path="/newsfeed" element={
                            <PrivateRoute>
                                <Newsfeed />
                            </ PrivateRoute>} />
                        <Route path="/item" element={
                            <PrivateRoute>
                                <Item />
                            </ PrivateRoute>}/>
                        <Route exact path="/admin" element={
                            <PrivateRoute>
                                <Admin />
                            </ PrivateRoute>}/>
                        <Route exact path="/login" element={<Login /> }/>
                        <Route path="/setup" element={<Setup />}/>
                        <Route path="*" element={<NotFound />} />
                    </Routes>
                </AuthProvider>
                </Router>
            </React.StrictMode>
        );
    }
}