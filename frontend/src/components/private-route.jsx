import React from "react";
import { Navigate, useLocation } from "react-router-dom";

import useAuth from "./auth";

function PrivateRoute({ children }) {
    const { authed } = useAuth();
    const location = useLocation();

    return authed ? children : <Navigate to="/login" state={location.pathname} />;
}
  

export default PrivateRoute;