import { ReloadOptions } from '@inertiajs/core';
import { ReactNode } from 'react';
interface WhenVisibleProps {
    children: ReactNode | (() => ReactNode);
    fallback: ReactNode | (() => ReactNode);
    data?: string | string[];
    params?: ReloadOptions;
    buffer?: number;
    as?: string;
    always?: boolean;
}
declare const WhenVisible: {
    ({ children, data, params, buffer, as, always, fallback }: WhenVisibleProps): ReactNode;
    displayName: string;
};
export default WhenVisible;
