import { ReactNode } from 'react';
interface DeferredProps {
    children: ReactNode | (() => ReactNode);
    fallback: ReactNode | (() => ReactNode);
    data: string | string[];
}
declare const Deferred: {
    ({ children, data, fallback }: DeferredProps): ReactNode;
    displayName: string;
};
export default Deferred;
