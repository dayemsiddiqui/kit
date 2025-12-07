declare const _default: {
    modal: null;
    listener: null;
    createIframeAndPage(html: Record<string, unknown> | string): {
        iframe: HTMLIFrameElement;
        page: HTMLElement;
    };
    show(html: Record<string, unknown> | string): void;
    hide(): void;
    hideOnEscape(event: KeyboardEvent): void;
};
export default _default;
