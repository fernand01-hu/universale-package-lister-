# Universal Package Lister

## Introduction
I use Linux Mint on my main working machine and have experience with several Linux distributions including Arch, Ubuntu, NixOS, and Fedora. While I am currently moving toward NixOS, I find it can be quite difficult to learn. I want a system that is stable, lightweight, and easy to use without it becoming too much of a hassle.

## The Problem
On most Linux distributions, when you install a package for a project, it often stays on your system indefinitely. If you don't manually clean up your binaries, your system can become very unorganized. While NixOS solves this, the learning curve feels like a barrier for many users.

## The Solution
The goal of this project is to create a universal tool that allows you to maintain packages by checking every package manager installed on your computer. It aims to simplify package management through a single interface.

## Key Features
* **Unified Updates:** Update everything on your system with one simple command.
* **System Snapshots:** Take a snapshot of your packages before performing unstable actions or large updates.
* **Duplicate Detection:** Verify if the same package is installed via two different package managers.
* **System Cleanup:** A dedicated tool to help you clean up your system easily.
* **Source Identification:** Check exactly which package manager a specific package comes from.
* **Development Consistency:** Use it as a multi-platform manager for large projects to avoid OS compatibility issues between developers.

## Project Roadmap
The initial development of this tool will focus on Ubuntu users, with plans to expand support to multiple Linux distributions in the future.
