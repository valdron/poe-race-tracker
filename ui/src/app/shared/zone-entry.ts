export class ZoneEntry {
    zone: string;
    duration_in_seconds: number;

    constructor(zone_name: string, seconds: number) {
        this.zone = zone_name;
        this.duration_in_seconds = seconds;
    }
}
