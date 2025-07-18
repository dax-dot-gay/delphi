import { ReactNode, use, useCallback, useState } from "react";
import {
    AuthContext,
    AuthContextAuthenticated,
    AuthContextConnected,
    AuthContextError,
} from "./types";
import { getStatus } from ".";

export function AuthProvider({
    promise,
    children,
}: {
    promise: Promise<
        AuthContextAuthenticated | AuthContextConnected | AuthContextError
    >;
    children?: ReactNode | ReactNode[];
}) {
    const initialStatus = use(promise);
    const [auth, setAuth] = useState(initialStatus);
    const refresh = useCallback(() => {
        getStatus().then(setAuth);
    }, [setAuth]);

    return (
        <AuthContext.Provider value={{ ...auth, refresh }}>
            {children}
        </AuthContext.Provider>
    );
}
