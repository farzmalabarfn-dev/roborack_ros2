# VIGIL — Autonomous Elderly Assistance Robot

VIGIL is a final-year robotics major project (Team 4) building an autonomous
mobile robot to assist elderly individuals — navigating indoor environments,
monitoring, and alerting caregivers when needed. The concept is aligned with
addressing challenges of an ageing population through accessible robotics.

## Status
🚧 In active development. This repo evolved from ROBORACK, a 5th-semester
mini-project (5-DOF differential-drive mobile manipulator), which serves as
the mechanical/electrical starting point for VIGIL.

**Completed:**
- Base robot description (URDF/SDF)
- ros2_control interface integration

**In progress:**
- Gazebo simulation environment
- SLAM and Nav2 navigation stack (Phase 1)

**Planned:**
- BLE indoor localization (Phase 2)
- Perception and alerting subsystems

## Hardware Stack
- Raspberry Pi 5 (8GB)
- Hailo-8L AI HAT+ (13 TOPS)
- RPLidar A1
- Arduino Mega 2560
- Cytron MDD10A motor driver
- Johnson 12V 200RPM quad-encoder motors
- ReSpeaker USB mic array
- LiFePO4 32650 4S1P battery

## Software Stack
- ROS 2 Jazzy on Ubuntu 24.04
- Gazebo
- Nav2
- Telegram Bot (alerting/interface)

## Team
Farz, Jazib Manzoor, Mohammed Nabeel Assim, Sreekuttan Biju
