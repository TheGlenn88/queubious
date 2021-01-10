export interface QueueDataInterface {
    position: number;
    progress: number;
    wait_time: string;
    last_updated: string;
    messages: Message[];
}

export interface Message {
    timestamp: string;
    message: string;
}
