SHELL=/usr/bin/env bash
DATA_PATH = ${HOME}/.0L
SWARM_TEMP = ${DATA_PATH}/swarm_temp
LOG=${DATA_PATH}/test-onboard.log
UNAME := $(shell uname)

NODE_ENV=test
TEST=y

ifndef SOURCE_PATH
SOURCE_PATH = ${HOME}/libra
endif
MAKE_FILE = ${SOURCE_PATH}/ol/integration-tests/test-onboard.mk

# alice
ifndef PERSONA
PERSONA=alice
endif

MNEM="talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse"

NUM_NODES = 4
EVE = 3DC18D1CF61FAAC6AC70E3A63F062E4B

ONBOARD_FILE=eve.fixed_recurring.account.json

START_TEXT = "To run the Libra CLI client"
SUCCESS_TEXT = "User transactions successfully relayed"

# account.json fixtures generated with:
# cargo r -p onboard -- --swarm-path ./whatever val --upstream-peer http://167.172.248.37/

test: swarm check-swarm send-tx check-tx check-account-created check-transfer stop

swarm:
	@echo Building Swarm
	rm -rf ${SWARM_TEMP}
	mkdir ${SWARM_TEMP}
	cd ${SOURCE_PATH} && cargo build -p libra-node -p cli
	cd ${SOURCE_PATH} && NODE_ENV=test TEST=y cargo run -p libra-swarm -- --libra-node ${SOURCE_PATH}/target/debug/libra-node -c ${SWARM_TEMP} -n ${NUM_NODES} &> ${LOG} &

stop:
	killall libra-swarm libra-node miner ol | true


init:
	@echo INIT
	cd ${SOURCE_PATH} && cargo r -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} init --source-path ${SOURCE_PATH}

tx:
	@echo TX
	cd ${SOURCE_PATH} && NODE_ENV=test TEST=y cargo r -p txs -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} create-validator -f ${SOURCE_PATH}/ol/fixtures/account/swarm/${ONBOARD_FILE}

resources:
	cd ${SOURCE_PATH} && cargo run -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} --account ${EVE} query --resources

balance:
	cd ${SOURCE_PATH} && cargo run -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} --account 3DC18D1CF61FAAC6AC70E3A63F062E4B query --balance

balance-bob:
	cd ${SOURCE_PATH} && cargo run -p ol -- --account 88E74DFED34420F2AD8032148280A84B --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} query --balance


check-swarm: 
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${START_TEXT} ${LOG} ; then \
				break; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

send-tx: 
	PERSONA=alice make -f ${MAKE_FILE} init
	PERSONA=alice make -f ${MAKE_FILE} tx &>> ${LOG} &

check-tx:
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${SUCCESS_TEXT} ${LOG} ; then \
				echo TX SUCCESS ; \
				break ; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

check-account-created: 
# checks if there is any mention of BOB's account as a payee on EVE's account
	PERSONA=alice make -f ${MAKE_FILE} resources | grep -e 'payee'

check-transfer:
# swarm accounts start with a balance of 4
	@while [[ ${NOW} -le ${END} ]] ; do \
			if PERSONA=alice make -f ${MAKE_FILE} balance-bob | grep -e '5'; then \
				echo TX SUCCESS ; \
				break ; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done
	