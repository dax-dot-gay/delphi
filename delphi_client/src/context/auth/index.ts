import { useContext } from "react";
import { AuthProvider } from "./AuthProvider";
import {
    AuthContext,
    AuthContextAuthenticated,
    AuthContextConnected,
    AuthContextError,
    AuthContextStatus,
    AuthContextType,
} from "./types";
import { Root, Session, UserProfile } from "../../api";

export type { AuthContextType };
export { AuthProvider };

export function useAuth(): AuthContextType {
    return useContext(AuthContext);
}

export function useAuthStatus(): AuthContextStatus {
    return useAuth().status;
}

export function useAuthUser(): UserProfile | null {
    const auth = useAuth();
    if (auth.status === "authenticated") {
        return auth.user;
    } else {
        return null;
    }
}

export function useAuthSession(): Session | null {
    const auth = useAuth();
    if (auth.status === "error") {
        return null;
    } else {
        return auth.session;
    }
}

export function useAuthError(): object | null {
    const auth = useAuth();
    if (auth.status === "error") {
        return auth.error;
    } else {
        return null;
    }
}

export function useAuthRefresh(): () => void {
    return useAuth().refresh;
}

export async function getStatus(): Promise<
    AuthContextConnected | AuthContextAuthenticated | AuthContextError
> {
    const result = await Root.getStatus();
    if (result.data) {
        if (result.data.user) {
            return {
                status: "authenticated",
                session: result.data.session,
                user: result.data.user,
            };
        } else {
            return {
                status: "connected",
                session: result.data.session,
            };
        }
    } else {
        return {
            status: "error",
            error: result.error,
        };
    }
}
