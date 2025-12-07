import { Component, Page, PageEvent, PageHandler, PageResolver, RouterInitParams, Visit } from './types';
declare class CurrentPage {
    protected page: Page;
    protected swapComponent: PageHandler<any>;
    protected resolveComponent: PageResolver;
    protected componentId: {};
    protected listeners: {
        event: PageEvent;
        callback: VoidFunction;
    }[];
    protected isFirstPageLoad: boolean;
    protected cleared: boolean;
    protected pendingDeferredProps: Pick<Page, 'deferredProps' | 'url' | 'component'> | null;
    init<ComponentType = Component>({ initialPage, swapComponent, resolveComponent, }: RouterInitParams<ComponentType>): this;
    set(page: Page, { replace, preserveScroll, preserveState, viewTransition, }?: {
        replace?: boolean;
        preserveScroll?: boolean;
        preserveState?: boolean;
        viewTransition?: Visit['viewTransition'];
    }): Promise<void>;
    setQuietly(page: Page, { preserveState, }?: {
        preserveState?: boolean;
    }): Promise<unknown>;
    clear(): void;
    isCleared(): boolean;
    get(): Page;
    merge(data: Partial<Page>): void;
    setUrlHash(hash: string): void;
    remember(data: Page['rememberedState']): void;
    swap({ component, page, preserveState, viewTransition, }: {
        component: Component;
        page: Page;
        preserveState: boolean;
        viewTransition: Visit['viewTransition'];
    }): Promise<unknown>;
    resolve(component: string): Promise<Component>;
    isTheSame(page: Page): boolean;
    on(event: PageEvent, callback: VoidFunction): VoidFunction;
    fireEventsFor(event: PageEvent): void;
}
export declare const page: CurrentPage;
export {};
