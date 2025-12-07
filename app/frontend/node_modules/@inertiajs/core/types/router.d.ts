import { RequestStream } from './requestStream';
import { ActiveVisit, ClientSideVisitOptions, Component, GlobalEvent, GlobalEventNames, GlobalEventResult, InFlightPrefetch, Page, PendingVisit, PendingVisitOptions, PollOptions, PrefetchedResponse, PrefetchOptions, ReloadOptions, RequestPayload, RouterInitParams, UrlMethodPair, VisitCallbacks, VisitHelperOptions, VisitOptions } from './types';
export declare class Router {
    protected syncRequestStream: RequestStream;
    protected asyncRequestStream: RequestStream;
    init<ComponentType = Component>({ initialPage, resolveComponent, swapComponent, }: RouterInitParams<ComponentType>): void;
    get<T extends RequestPayload = RequestPayload>(url: URL | string | UrlMethodPair, data?: T, options?: VisitHelperOptions<T>): void;
    post<T extends RequestPayload = RequestPayload>(url: URL | string | UrlMethodPair, data?: T, options?: VisitHelperOptions<T>): void;
    put<T extends RequestPayload = RequestPayload>(url: URL | string | UrlMethodPair, data?: T, options?: VisitHelperOptions<T>): void;
    patch<T extends RequestPayload = RequestPayload>(url: URL | string | UrlMethodPair, data?: T, options?: VisitHelperOptions<T>): void;
    delete<T extends RequestPayload = RequestPayload>(url: URL | string | UrlMethodPair, options?: Omit<VisitOptions<T>, 'method'>): void;
    reload<T extends RequestPayload = RequestPayload>(options?: ReloadOptions<T>): void;
    remember(data: unknown, key?: string): void;
    restore<T = unknown>(key?: string): T | undefined;
    on<TEventName extends GlobalEventNames>(type: TEventName, callback: (event: GlobalEvent<TEventName>) => GlobalEventResult<TEventName>): VoidFunction;
    cancel(): void;
    cancelAll(): void;
    poll(interval: number, requestOptions?: ReloadOptions, options?: PollOptions): {
        stop: VoidFunction;
        start: VoidFunction;
    };
    visit<T extends RequestPayload = RequestPayload>(href: string | URL | UrlMethodPair, options?: VisitOptions<T>): void;
    getCached(href: string | URL | UrlMethodPair, options?: VisitOptions): InFlightPrefetch | PrefetchedResponse | null;
    flush(href: string | URL | UrlMethodPair, options?: VisitOptions): void;
    flushAll(): void;
    flushByCacheTags(tags: string | string[]): void;
    getPrefetching(href: string | URL | UrlMethodPair, options?: VisitOptions): InFlightPrefetch | PrefetchedResponse | null;
    prefetch(href: string | URL | UrlMethodPair, options?: VisitOptions, prefetchOptions?: Partial<PrefetchOptions>): void;
    clearHistory(): void;
    decryptHistory(): Promise<Page>;
    resolveComponent(component: string): Promise<Component>;
    replace<TProps = Page['props']>(params: ClientSideVisitOptions<TProps>): void;
    replaceProp<TProps = Page['props']>(name: string, value: unknown | ((oldValue: unknown, props: TProps) => unknown), options?: Pick<ClientSideVisitOptions, 'onError' | 'onFinish' | 'onSuccess'>): void;
    appendToProp<TProps = Page['props']>(name: string, value: unknown | unknown[] | ((oldValue: unknown, props: TProps) => unknown | unknown[]), options?: Pick<ClientSideVisitOptions, 'onError' | 'onFinish' | 'onSuccess'>): void;
    prependToProp<TProps = Page['props']>(name: string, value: unknown | unknown[] | ((oldValue: unknown, props: TProps) => unknown | unknown[]), options?: Pick<ClientSideVisitOptions, 'onError' | 'onFinish' | 'onSuccess'>): void;
    push<TProps = Page['props']>(params: ClientSideVisitOptions<TProps>): void;
    protected clientVisit<TProps = Page['props']>(params: ClientSideVisitOptions<TProps>, { replace }?: {
        replace?: boolean;
    }): void;
    protected getPrefetchParams(href: string | URL | UrlMethodPair, options: VisitOptions): ActiveVisit;
    protected getPendingVisit(href: string | URL | UrlMethodPair, options: VisitOptions, pendingVisitOptions?: Partial<PendingVisitOptions>): PendingVisit;
    protected getVisitEvents(options: VisitOptions): VisitCallbacks;
    protected loadDeferredProps(deferred: Page['deferredProps']): void;
}
