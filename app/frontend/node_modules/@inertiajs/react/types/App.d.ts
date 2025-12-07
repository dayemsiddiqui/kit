import { HeadManagerOnUpdateCallback, HeadManagerTitleCallback, Page, PageProps } from '@inertiajs/core';
import { FunctionComponent, ReactNode } from 'react';
import { ReactComponent } from './types';
export interface InertiaAppProps<SharedProps extends PageProps = PageProps> {
    children?: (options: {
        Component: ReactComponent;
        props: PageProps;
        key: number | null;
    }) => ReactNode;
    initialPage: Page<SharedProps>;
    initialComponent?: ReactComponent;
    resolveComponent?: (name: string) => ReactComponent | Promise<ReactComponent>;
    titleCallback?: HeadManagerTitleCallback;
    onHeadUpdate?: HeadManagerOnUpdateCallback;
}
export type InertiaApp = FunctionComponent<InertiaAppProps>;
declare function App<SharedProps extends PageProps = PageProps>({ children, initialPage, initialComponent, resolveComponent, titleCallback, onHeadUpdate, }: InertiaAppProps<SharedProps>): import("react").FunctionComponentElement<import("react").ProviderProps<import("@inertiajs/core").HeadManager | null>>;
declare namespace App {
    var displayName: string;
}
export default App;
