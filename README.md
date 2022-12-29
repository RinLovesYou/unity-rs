<!-- Improved compatibility of back to top link: See: https://github.com/RinLovesYou/unity-rs/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/RinLovesYou/unity-rs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">unity-rs</h3>

  <p align="center">
    A Library for interacting with unity at runtime
    <br />
    <a href="https://github.com/RinLovesYou/unity-rs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/RinLovesYou/unity-rs">View Demo</a>
    ·
    <a href="https://github.com/RinLovesYou/unity-rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/RinLovesYou/unity-rs/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
## About

THIS REPO IS [MOVING!](https://github.com/RinLovesYou/Ferrex/tree/master/unity)

Unity-rs provides an abstracted api to interact with Unity Games at runtime. It supports both Mono & Il2cpp, and will automatically detect which the game is running in.<br>
By using this library, you will mostly not have to concern yourself with the specifics of the implementation, and can just interact with the game.
Of course, we also expose internal things, such as function pointers to mono/il2cpp functions, in case you need to hook them.


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

This project is mostly aimed at the use case for modding, so please take care of injecting a rust cdylib into the unity game.

### Installation

Simply add it to your `Cargo.toml`:
```
unity-rs = { git = "https://github.com/RinLovesYou/unity-rs.git" }
```
The Project is currently not in a state where i am confident to publish to Crates.io

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Under Construction

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] Mono/Old Mono
- [ ] Il2cpp
- [ ] Custom Dobby wrapper, with integration

See the [open issues](https://github.com/RinLovesYou/unity-rs/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the Apache-2.0 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

RinLovesYou - [@does_rin](https://twitter.com/does_rin) - rin@pepsi.group - Rin#6969 (Discord)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/RinLovesYou/unity-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/RinLovesYou/unity-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/RinLovesYou/unity-rs.svg?style=for-the-badge
[forks-url]: https://github.com/RinLovesYou/unity-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/RinLovesYou/unity-rs.svg?style=for-the-badge
[stars-url]: https://github.com/RinLovesYou/unity-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/RinLovesYou/unity-rs.svg?style=for-the-badge
[issues-url]: https://github.com/RinLovesYou/unity-rs/issues
[license-shield]: https://img.shields.io/github/license/RinLovesYou/unity-rs.svg?style=for-the-badge
[license-url]: https://github.com/RinLovesYou/unity-rs/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/sarah-codenz-17219a198/
