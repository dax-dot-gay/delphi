import { createContext } from "react";
import { Session, UserProfile } from "../../api";

export type AuthContextConnected = {
    status: "connected";
    session: Session;
};

export type AuthContextAuthenticated = {
    status: "authenticated";
    session: Session;
    user: UserProfile;
};

export type AuthContextError = {
    status: "error";
    error: any;
};

export type AuthContextType = (
    | AuthContextConnected
    | AuthContextAuthenticated
    | AuthContextError
) & { refresh: () => void };

export type AuthContextStatus = AuthContextType["status"];

export const AuthContext = createContext<AuthContextType>({
    status: "error",
    error: "Context not initialized.",
    refresh: () => {},
});
