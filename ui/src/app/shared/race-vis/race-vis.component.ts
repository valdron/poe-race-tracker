import { Component, OnInit, ViewEncapsulation, OnChanges, ElementRef, ViewChild, Input } from '@angular/core';
import d3 = require('d3');

@Component({
  selector: 'app-race-vis',
  templateUrl: './race-vis.component.html',
  styleUrls: ['./race-vis.component.css'],
  encapsulation: ViewEncapsulation.None
})
export class RaceVisComponent implements OnInit, OnChanges {

  @ViewChild('chart') private chartContainer: ElementRef;
  @Input() private data: Array<any>;
  private margin: any = { top: 20, bottom: 20, left: 20, right: 20 };
  private chart: any;
  private width: number;
  private height: number;
  private xScale: any;
  private yScale: any;
  private colors: any;
  private xAxis: any;
  private yAxis: any;

  constructor() { }

  ngOnInit() {
    this.createChart();
    if (this.data) {
      this.updateChart();
    }
  }

  ngOnChanges() {
    if (this.chart) {
      this.updateChart();
    }
  }

  createChart() {
  }

  updateChart() {
  }

}
