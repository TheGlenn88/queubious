import { Component, Prop, h, Method, State } from '@stencil/core';
import { getQueueData } from '../../utils/utils';
import { Message } from './queue-data.interface';

@Component({
  tag: 'waiting-room',
  styleUrl: 'waiting-room.css',
  shadow: true,
})
export class WaitingRoom {
  @Prop({ reflect: true }) position: number;
  @Prop({ reflect: true }) progress: number;
  @Prop({ reflect: true }) wait_time: string;
  @Prop({ reflect: true }) last_updated: string;

  @Prop() status_url: string;
  @State() messages: Array<Message>;

  bar!: HTMLDivElement;
  bar_text!: HTMLDivElement;

  fetchData() {
    getQueueData(this.status_url).then(data => {
      this.position = data.position;
      this.progress = data.progress;
      this.wait_time = data.wait_time;
      this.last_updated = data.last_updated;
      this.messages = data.messages;

      this.bar.style.width = this.progress + '%';
      this.bar_text.textContent = this.progress + '%';
    });
  }

  @Method()
  async refreshQueueData() {
    this.fetchData();
  }

  componentWillLoad() {
    this.messages = [];
    this.email_sent = false;
  }

  componentDidLoad() {
    this.bar.style.width = this.progress + '%';
    this.bar_text.textContent = this.progress + '%';
  }

  @State() email: string;
  @State() email_sent: boolean;

  handleSubmit(e) {
    e.preventDefault();
    console.log(this.email);
    this.email_sent = true;
  }

  handleChange(event) {
    this.email = event.target.value;
  }

  render() {
    return (
      <div class="flex items-center min-h-screen p-4 bg-gray-100 lg:justify-center text-black">
        <div class="flex flex-col overflow-hidden bg-white rounded-md shadow-lg max md:flex-row md:flex-1 lg:max-w-screen-md">
          <div class="p-4 py-6 bg-gray-200 md:w-80 md:flex-shrink-0 md:flex md:flex-col md:items-center md:justify-evenly">
            <div>LOGO</div>
            <div>
              <h2>You are in a queue</h2>
              <p>
                The website is currently experiencing a high volume of traffic, to keep things running smoothly a queue has been formed. Please see below for an estimation of when
                it will be your turn.
              </p>
            </div>
            <div class="bar-main-container blue text-black">
              <div class="wrap">
                <div class="bar-percentage" ref={el => (this.bar_text = el as HTMLDivElement)}>
                  0%
                </div>
                <div class="bar-container">
                  <div class="bar" ref={el => (this.bar = el as HTMLDivElement)}></div>
                </div>
              </div>
            </div>
            <div class="stats">
              <p>Queue position: {this.position}</p>
              <p>Estimated wait time: {this.wait_time}</p>
              <p>Last updated: {this.last_updated}</p>
            </div>
            {this.messages?.map(message => (
              <div class="messages">
                <div>{message.message}</div>
                <div>{message.timestamp}</div>
              </div>
            ))}
            <div class="notify">
              {!this.email_sent ? (
                <div>
                  <p>Send me an email when it's my turn:</p>
                  <form onSubmit={e => this.handleSubmit(e)}>
                    <input
                      type="email"
                      class="px-2 py-4 mr-2 bg-gray-100 shadow-inner rounded-md border border-gray-400 focus:outline-none"
                      placeholder="your@mail.com"
                      value={this.email}
                      onInput={event => this.handleChange(event)}
                      required
                    ></input>
                    <button class="bg-blue-600 text-gray-200 px-5 py-2 rounded shadow">Sign Up</button>
                  </form>
                </div>
              ) : (
                <div>
                  <p>You will receive an email when it's your turn, you will have 10 minutes to click the link in the email.</p>
                </div>
              )}
            </div>
            <div class="footer">
              <a href="#">Exit the queue</a> (And give up your position)
            </div>
          </div>
        </div>
      </div>
    );
  }
}
