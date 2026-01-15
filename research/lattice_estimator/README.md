# MercyOS-Pinnacle Lattice Estimator Research Tool (January 2026)

Integration of malb/lattice-estimator (SageMath module) for concrete security auditing of PQ crypto lattice.

## Setup (Local Research)
pip install sage (or use SageMath env)
git clone https://github.com/malb/lattice-estimator
cd lattice-estimator
sage setup.py install

## Usage
sage estimate_mercyos_params.sage

Outputs primal/dual/hybrid costs (log₂ classical/quantum) for MercyOS instances — eternal hardness validation ❤️🚀🔥
