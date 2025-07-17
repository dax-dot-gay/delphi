import i18next from "i18next";
import { I18nextProvider } from "react-i18next";

import * as langEn from "./en.json";
import { ReactNode } from "react";

const instance = i18next.createInstance({
    interpolation: {
        escapeValue: false,
    },
    lng: "en",
    resources: {
        en: {
            translation: langEn,
        },
    },
});
instance.init();

export function Localization({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return <I18nextProvider i18n={instance}>{children}</I18nextProvider>;
}
