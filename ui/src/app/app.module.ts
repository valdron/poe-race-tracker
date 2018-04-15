import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';


import { AppComponent } from './app.component';
import { RaceVisComponent } from './shared/race-vis/race-vis.component';


@NgModule({
  declarations: [
    AppComponent,
    RaceVisComponent
  ],
  imports: [
    BrowserModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
