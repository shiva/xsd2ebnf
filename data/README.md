This appendix contains the actual files that constitute the Space Surveillance Ontology.  The
files were created with the April 2000 draft version of the XML Schema specification.
Although a newer version of the specification was published during our development effort,
we used this version because the validator we used was compliant with the April 2000
specification working draft.  Each file is contained in a separate subsection.  The files
included in this appendix, along with a brief description of each, is listed below.

# SpaceSurveillance.xsd:  Schema for space surveillance data sets. This is a 
  container element for the space surveillance elements.
# SpaceSurvGlobals.xsd:  Schema file that contains global attribute declarations 
  and type definitions for the space surveillance namespace.
# Satellite.xsd:  Schema for a satellite from a space surveillance perspective.
# LaunchSiteDefintions.doc:  Text file that contains satellite launch site short 
  names and the associated satellite launch site long name.  This file is referenced 
  in the documentation annotation for the LaunchSite element in the satellite schema.
# Sensor.xsd:  Schema for a sensor from a space surveillance perspective. Presumes a 
  ground-based sensor.
# SensorTasking.xsd:  Schema that describes the priority and amount of data a sensor 
  is to collect on each tasked satellite.
# SatObservation.xsd:  Schema for a satellite observation by a Space Surveillance Network sensor.
# SatElset.xsd:  Schema for satellite general perturbations orbital element set.
# ss.xml:  Instance document that contains instances of the SpaceSurveillanceData 
  elements:  Satellite, Sensor, SatObservation, and SatelliteElementSet.
# tasking.xml:  Instance document that contains an instance of the SensorTask element
