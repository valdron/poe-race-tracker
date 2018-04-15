import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RaceVisComponent } from './race-vis.component';

describe('RaceVisComponent', () => {
  let component: RaceVisComponent;
  let fixture: ComponentFixture<RaceVisComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RaceVisComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RaceVisComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
