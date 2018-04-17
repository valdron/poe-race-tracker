export class ZoneEntry {
    name: string;
    seconds_after_start: number;

    constructor(zone_name: string, seconds: number) {
        this.name = zone_name;
        this.seconds_after_start = seconds;
    }
}
