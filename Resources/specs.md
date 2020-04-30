# Specs

This page is for the technical details associated with GPS modules. 

Data sentences to and from the GPS module are formatted as such: $Header,arg1,arg2,...,argN*checksum\r\n. 

# NMEA data formats

The GPS uses NMEA 0183 protocol. 

There are 6 data formats that the GPS can send: 





# Sending the gps commands: PMTK

Adafruit Ultimate GPS is MTK3339 chip. 

Adafruit Mini GPS PA1010D Module is MTK3333 chip. 



| Output sentences | PA6B (MTK3329) | PA6C (MTK3339) | PA6H (MTK3339) |
| ---------------- | -------------- | -------------- | -------------- |
| GGA              | y              | y              | y              |
| GSA              | y              | y              | y              |
| GSV              | y              | y              | y              |
| RMC              | y              | y              | y              |
| VTG              | y              | y              | y              |

### Pmtk sentence compatibility 

When power to the module is removed, all settings return to default. 

Ptmk_ack is the sentence to acknowledge a sent command. 

So send 010 and get back a response. PMTK001,cmd that is being acknowledged,flag(0,1,2,3: invalid, unsupported command, valid command but action failed, valid command and succeeded)

| pmtk sentence                      | MT3318 | MT3329 | MT3339 | Description |
| ---------------------------------- | ------ | ------ | ------ | ----------- |
| 001 pmtk_ack                       | y      | y      | y      |             |
| 010 PMTK_SYS_MSG                   | y      | y      | y      |             |
| 011 PMTK_TXT_MSG                   | y      | y      | y      |             |
| 101 PMTK_CMD_HOT_START             | y      | y      | y      |             |
| 102 PMTK_CMD_WARM_START            | y      | y      | y      |             |
| 103 PMTK_CMD_COLD_START            | y      | y      | y      |             |
| 104 PMTK_CMD_FULL_COLD_START       | y      | y      | y      |             |
| 220 PMTK_SET_NMEA_UPDATERATE       | y      | y      | y      |             |
| 251 PMTK_SET_NMEA_BAUDRATE         | y      | y      | y      |             |
| 301 PMTK_API_SET_DGPS_MODE         | y      | y      | y      |             |
| 401 PMTK_API_Q_DGPS_MODE           | y      | y      | y      |             |
| 501 PMTK_API_DT_DGPS_MODE          | y      | y      | y      |             |
| 313 PMTK_API_SET_SBAS_ENABLED      | y      | y      | y      |             |
| 413 PMTK_API_Q_SBAS_ENABLED        | y      | y      | y      |             |
| 513 PMTK_DT_SBAS_ENABLED           | y      | y      | y      |             |
| 314 PMTK_API_SET_NMEA_OUTPUT       | y      | y      | y      |             |
| 414 PMTK_API_Q_NMEA_OUTPUT         | y      | y      | y      |             |
| 514 PMTK_API_DT_NMEA_OUTPUT        | y      | y      | y      |             |
| 319 PMTK_API_SET_SBAS_Mode         | y      | y      | y      |             |
| 419 PMTK_API_Q_SBAS_Mode           | y      | y      | y      |             |
| 519 PMTK_API_DT_SBAS_Mode          | y      | y      | y      |             |
| 605 PMTK_Q_RELEASE                 | y      | y      | y      |             |
| 705 PMTK_DT_RELEASE                | y      | y      | y      |             |
| 607 PMTK_Q_EPO_INFO                | y      | y      | y      |             |
| 707 PMTK_DT_EPO_INFO               | y      | y      | y      |             |
| 127 PMTK_CMD_CLEAR_EPO             | y      | y      | y      |             |
| 397 PMTK_SET_Nav Speed threshold   | y      | y      | n      |             |
| 386 PMTK_SET_Nav Speed threshold   | n      | n      | y      |             |
| 447 PMTK_Q_Nav_Threshold           | y      | y      | y      |             |
| 527 PMTK_DT_Nav_Threshold          | y      | y      | y      |             |
| 161 PMTK_CMD_STANDBY_MODE          | n      | n      | y      |             |
| 223 PMTK_SET_AL_DEE_CFG            | n      | n      | y      |             |
| 225 PMTK_CMD_PERIODIC_MODE         | n      | n      | y      |             |
| 286 PMTK_CMD_AIC_MODE              | n      | n      | y      |             |
| 869 PMTK_CMD_EASY_ENABLE           | n      | n      | y      |             |
| 187 PMTK_LOCUS_CONFIG              | n      | n      | y      |             |
| 330 PMTK_API_SET_DATUM             | y      | y      | y      |             |
| 430 PMTK_API_Q_DATUM               | y      | y      | y      |             |
| 530 PMTK_API_DT_DATUM              | y      | y      | y      |             |
| 351 PMTK_API_SET_SUPPORT_QZSS_NMEA | n      | n      | y      |             |
| 352 PMTK_API_SET_STOP_QZSS         | n      | n      | y      |             |
|                                    |        |        |        |             |























