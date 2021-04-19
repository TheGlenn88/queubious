import { Component, Prop, h, Method, getAssetPath, State } from '@stencil/core';
import { getQueueData } from '../../utils/utils';
import { Message } from './queue-data.interface';

@Component({
  tag: 'waiting-room',
  styleUrl: 'waiting-room.css',
  shadow: true,
  assetsDirs: ['assets'],
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
      <div class="font-sans min-h-screen bg-gray-100 py-6 flex flex-col justify-center sm:py-12">
        <div class="relative py-3 sm:max-w-xl sm:mx-auto">
          <div class="absolute inset-0 bg-gradient-to-r from-cyan-400 to-light-blue-500 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl"></div>
          <div class="relative px-4 py-10 bg-white shadow-lg sm:rounded-3xl sm:p-20">
            <div class="max-w-md mx-auto">
              <div>
                <img src={getAssetPath('./assets/logo.svg')} class="h-7 sm:h-8" />
              </div>
              <div class="divide-y divide-gray-200">
                <div class="py-8 text-base leading-6 space-y-4 text-gray-700 sm:text-lg sm:leading-7">
                  <p> ||website|| is experiencing a higher than usual traffic, you are now in a queue.</p>
                  <div class="w-full h-4 bg-gray-400 rounded-full mt-3">
                    <div class="w-3/4 h-full text-center text-xs text-white bg-green-500 rounded-full">75%</div>
                  </div>
                  <ul>
                    <li>Your position:</li>
                    <li>Estimated Wait Time:</li>
                    <li>Last updated:</li>
                  </ul>
                </div>
                {this.messages?.map(message => (
                  <div class="messages">
                    <div>{message.message}</div>
                    <div>{message.timestamp}</div>
                  </div>
                ))}
                <div class="pt-6">
                  {!this.email_sent ? (
                    <div>
                      <div class="pt-6 text-base leading-6 font-bold sm:text-lg sm:leading-7">
                        <p>Prefert not to wait?</p>
                      </div>
                      <div>
                        <p>Send me an email when it's my turn:</p>
                        <form onSubmit={e => this.handleSubmit(e)}>
                          <input
                            type="email"
                            class="px-5 py-2 bg-gray-100shadow-inner rounded-md border border-gray-400 focus:outline-none"
                            placeholder="your@mail.com"
                            value={this.email}
                            onInput={event => this.handleChange(event)}
                            required
                          ></input>
                          <button class="bg-red-500 ml-2 text-gray-200 px-5 py-2 rounded shadow">Sign Up</button>
                        </form>
                      </div>
                    </div>
                  ) : (
                    <div>
                      <p>Thank you</p>
                    </div>
                  )}
                  <div>
                    <a href="#">Exit the queue</a> (And give up your position)
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
}
