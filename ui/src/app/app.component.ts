import { Component } from '@angular/core';
import { RaceRun } from './shared/race-run';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'app';
  chartData: RaceRun = {
    id: 1,
    duration_in_seconds: 1079,
    zones: [
      {
        name: 'Lioneye\'s Watch',
        seconds_after_start: 77
      },
      {
        name: 'The Coast',
        seconds_after_start: 166
      },
      {
        name: 'The Tidal Island',
        seconds_after_start: 212
      },
      {
        name: 'Lioneye\'s Watch',
        seconds_after_start: 280
      },
      {
        name: 'The Coast',
        seconds_after_start: 322
      },
      {
        name: 'The Mud Flats',
        seconds_after_start: 329
      },
      {
        name: 'The Submerged Passage',
        seconds_after_start: 407
      },
      {
        name: 'The Ledge',
        seconds_after_start: 501
      },
      {
        name: 'The Climb',
        seconds_after_start: 670
      },
      {
        name: 'The Lower Prison',
        seconds_after_start: 777
      },
      {
        name: 'Lioneye\'s Watch',
        seconds_after_start: 792
      },
      {
        name: 'The Lower Prison',
        seconds_after_start: 833
      },
      {
        name: 'The Upper Prison',
        seconds_after_start: 892
      },
      {
        name: 'Lioneye\'s Watch',
        seconds_after_start: 1075
      }
    ],
    levels: [
      {
        level: 2,
        seconds_after_start: 67
      },
      {
        level: 3,
        seconds_after_start: 396
      },
      {
        level: 4,
        seconds_after_start: 480
      },
      {
        level: 5,
        seconds_after_start: 547
      },
      {
        level: 6,
        seconds_after_start: 749
      },
      {
        level: 7,
        seconds_after_start: 911
      },
      {
        level: 8,
        seconds_after_start: 999
      }
    ]
  };
}
