import { Component, OnInit, ViewEncapsulation, OnChanges, ElementRef, ViewChild, Input } from '@angular/core';

import { RaceRun } from '../race-run';
import * as d3 from 'd3';

@Component({
  selector: 'app-race-vis',
  templateUrl: './race-vis.component.html',
  styleUrls: ['./race-vis.component.css'],
  encapsulation: ViewEncapsulation.None
})
export class RaceVisComponent implements OnInit, OnChanges {

  @ViewChild('chart') private chartContainer: ElementRef;
  @Input() private data: RaceRun;
  private margin: any = { top: 20, bottom: 20, left: 20, right: 20 };
  private chart: any;
  private width: number;
  private height: number;
  private xDomain: number[];
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
    const element = this.chartContainer.nativeElement;
    this.width = element.offsetWidth - this.margin.left - this.margin.right;
    this.height = element.offsetHeight - this.margin.top - this.margin.bottom;
    const svg = d3.select(element).append('svg')
      .attr('width', element.offsetWidth)
      .attr('height', element.offsetHeight);
    this.chart = svg.append('g')
      .attr('class', 'race')
      .attr('transform', `translate(${this.margin.left}, ${this.margin.top})`);

    this.xDomain = [0, this.data.duration_in_seconds * 1000];
    const yDomain = [0, 0];
    this.xScale = d3.scaleTime().domain(this.xDomain).range([0, this.width]);
    this.colors = d3.scaleLinear().domain([0, this.data.levels.length + this.data.zones.length]).range(<any[]>['red', 'blue']);
    this.xAxis = svg.append('g')
      .attr('class', 'axis axis-x')
      .attr('transform', `translate(${this.margin.left}, ${this.margin.top + this.height})`)
      .call(d3.axisBottom(this.xScale).ticks(d3.timeMinute, 1).tickFormat(d3.timeFormat('%Hh %Mm %Ss')));
  }

  updateChart() {
    this.xDomain = [0, this.data.duration_in_seconds * 1000];
    this.xScale = d3.scaleTime().domain(this.xDomain).range([0, this.width]);
    this.colors = d3.scaleLinear().domain([0, this.data.levels.length + this.data.zones.length]).range(<any[]>['red', 'blue']);
    this.xAxis.transition().call(d3.axisBottom(this.xScale));


    const zonesUpdate = this.chart.selectAll('.zone')
      .data(this.data.zones);

    // remove exiting bars
    zonesUpdate.exit().remove();

    this.chart.selectAll('.zone').transition()
      .attr('x', d => this.xScale(d.seconds_after_start * 1000))
      .attr('y', d => this.margin.top)
      .attr('width', d => this.xScale(30 * 1000))
      .attr('height', d => this.height - this.margin.bottom)
      .style('fill', (d, i) => this.colors(i));

    zonesUpdate
      .enter()
      .append('rect')
      .attr('class', 'zone')
      .attr('x', d => this.xScale(d.seconds_after_start * 1000))
      .attr('y', d => this.margin.top)
      .attr('width', this.xScale(30 * 1000))
      .attr('height', this.height - this.margin.bottom)
      .style('fill', (d, i) => this.colors(i));
  }

}
