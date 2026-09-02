export interface EventStore {
    get(id: string): unknown;
}
