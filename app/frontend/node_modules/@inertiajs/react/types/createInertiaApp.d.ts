import { CreateInertiaAppOptionsForCSR, CreateInertiaAppOptionsForSSR, InertiaAppSSRResponse, PageProps } from '@inertiajs/core';
import { ReactElement } from 'react';
import { renderToString } from 'react-dom/server';
import { InertiaAppProps, type InertiaApp } from './App';
import { ReactComponent, ReactInertiaAppConfig } from './types';
export type SetupOptions<ElementType, SharedProps extends PageProps> = {
    el: ElementType;
    App: InertiaApp;
    props: InertiaAppProps<SharedProps>;
};
type ComponentResolver = (name: string) => ReactComponent | Promise<ReactComponent> | {
    default: ReactComponent;
} | unknown;
type InertiaAppOptionsForCSR<SharedProps extends PageProps> = CreateInertiaAppOptionsForCSR<SharedProps, ComponentResolver, SetupOptions<HTMLElement, SharedProps>, void, ReactInertiaAppConfig>;
type InertiaAppOptionsForSSR<SharedProps extends PageProps> = CreateInertiaAppOptionsForSSR<SharedProps, ComponentResolver, SetupOptions<null, SharedProps>, ReactElement, ReactInertiaAppConfig> & {
    render: typeof renderToString;
};
export default function createInertiaApp<SharedProps extends PageProps = PageProps>(options: InertiaAppOptionsForCSR<SharedProps>): Promise<void>;
export default function createInertiaApp<SharedProps extends PageProps = PageProps>(options: InertiaAppOptionsForSSR<SharedProps>): Promise<InertiaAppSSRResponse>;
export {};
