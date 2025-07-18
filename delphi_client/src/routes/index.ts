import { createBrowserRouter } from "react-router";
import { Layout } from "./layout/Layout";
import { AuthView } from "./auth/AuthView";

export const router = createBrowserRouter([
    {
        path: "/",
        Component: Layout,
        children: [
            {
                path: "/auth",
                Component: AuthView,
            },
        ],
    },
]);
