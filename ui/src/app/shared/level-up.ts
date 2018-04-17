export class LevelUp {
    level: number;
    seconds_after_start: number;

    constructor(level: number, seconds: number) {
        this.level = level;
        this.seconds_after_start = seconds;
    }
}
